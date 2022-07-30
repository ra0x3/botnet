import {Flex, Text, Image} from 'rebass';

interface LoadingProps {
  sx?: any;
}
const Loading = ({sx}: LoadingProps) => {
  return (
    <Flex
      justifyContent={'center'}
      alignItems={'center'}
      sx={{border: '1px solid red', width: 300, height: 300, ...sx}}
    >
      <Image src="/img/loading.gif" alt="loading" />
    </Flex>
  );
};

export default Loading;
